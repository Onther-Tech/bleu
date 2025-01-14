pub mod tx_batch {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismTxBatch, OptimismTxBatchSummary};
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_batches_ctc;
    use crate::schema::optimism::optimism_batches_ctc::columns::*;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_latest_batch_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxBatchSummary>, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = web::block(move || {
            optimism_batches_ctc::table.select((batch_index, l1_tx_hash, batch_size, batch_timestamp))
                .order(optimism_batches_ctc_id.desc())
                .limit(10)
                .load::<OptimismTxBatchSummary>(&conn)
        }).await?;
        Ok(batch_summary)
    }

    pub async fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismTxBatch, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = web::block(move || {
            optimism_batches_ctc::table.filter(batch_index.eq(index.to_string()))
                .first::<OptimismTxBatch>(&conn)
        }).await?;
        Ok(batch_summary)
    }

    pub async fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismTxBatch>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_batch = web::block(move || {
            optimism_batches_ctc::table.into_boxed()
                .order(optimism_batches_ctc_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_batch)
    }

    pub async fn find_latest_tx_batch(conn: PgConn) -> Result<OptimismTxBatch, ExpectedError> {
        let latest_tx_batch = web::block(move || {
            optimism_batches_ctc::table.into_boxed()
                .order(optimism_batches_ctc_id.desc())
                .first::<OptimismTxBatch>(&conn)
        }).await?;
        Ok(latest_tx_batch)
    }
}

pub mod tx {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismBlockTxDetail, OptimismBlockTxExtended, OptimismBlockTxPaging, OptimismTxSummary};
    use crate::repository::ethereum::ethereum_log::find_by_queue_index;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::{optimism_block_txs, optimism_batches_scc_roots, optimism_tx_receipts};
    use crate::schema::optimism::optimism_block_txs::columns::*;
    use crate::schema::optimism::optimism_batches_ctc_txs;

    pub async fn find_latest_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let tx_summary = web::block(move || {
            optimism_block_txs::table.select((hash, from_address, to_address, value, l1_timestamp))
                .order(optimism_block_txs_id.desc())
                .limit(10)
                .load::<OptimismTxSummary>(&conn)
        }).await?;
        Ok(tx_summary)
    }

    pub async fn find_tx_by_hash(pool: web::Data<Pool>, tx_hash: String) -> Result<OptimismBlockTxDetail, ExpectedError> {
        Ok(find_tx_detail_by_condition(pool, Some(tx_hash), None).await?)
    }

    pub async fn find_tx_by_index(pool: web::Data<Pool>, batch_tx_index: i64) -> Result<OptimismBlockTxDetail, ExpectedError> {
        Ok(find_tx_detail_by_condition(pool, None, Some(batch_tx_index)).await?)
    }

    pub async fn find_tx_detail_by_condition(pool: web::Data<Pool>, tx_hash: Option<String>, batch_tx_index: Option<i64>) -> Result<OptimismBlockTxDetail, ExpectedError> {
        let conn = pool.get()?;
        let tx_ext: OptimismBlockTxExtended = web::block(move || {
            let query = optimism_block_txs::table
                .left_outer_join(optimism_batches_ctc_txs::table.on(optimism_block_txs::columns::index.eq(optimism_batches_ctc_txs::columns::index)))
                .left_outer_join(optimism_batches_scc_roots::table.on(optimism_block_txs::columns::index.eq(optimism_batches_scc_roots::columns::index)))
                .left_outer_join(optimism_tx_receipts::table.on(optimism_block_txs::columns::hash.eq(optimism_tx_receipts::columns::tx_hash)))
                .select((
                    optimism_block_txs::all_columns,
                    optimism_batches_ctc_txs::columns::batch_index.nullable(),
                    optimism_batches_ctc_txs::columns::l1_tx_hash.nullable(),
                    optimism_batches_scc_roots::columns::batch_index.nullable(),
                    optimism_batches_scc_roots::columns::l1_tx_hash.nullable(),
                    optimism_tx_receipts::columns::status.nullable(),
                    optimism_tx_receipts::columns::gas_used.nullable(),
                    optimism_tx_receipts::columns::l1_fee.nullable(),
                    optimism_tx_receipts::columns::l1_fee_scalar.nullable(),
                    optimism_tx_receipts::columns::l1_gas_price.nullable(),
                    optimism_tx_receipts::columns::l1_gas_used.nullable(),
                    optimism_tx_receipts::columns::contract_address.nullable()
                ));
            if tx_hash.is_some() {
                query.filter(hash.eq(tx_hash.unwrap().clone())).first::<OptimismBlockTxExtended>(&conn)
            } else {
                query.filter(optimism_block_txs::index.eq(batch_tx_index.unwrap().to_string())).first::<OptimismBlockTxExtended>(&conn)
            }
        }).await?;
        let conn = pool.get()?;
        let l1_tx_hash: Option<String> = match tx_ext.is_l1_origin() && tx_ext.is_queue_index_exist() {
            true => {
                match find_by_queue_index(conn, tx_ext.get_queue_index().unwrap()).await {
                    Ok(tx_log) => tx_log.get_tx_hash(),
                    Err(_) => None,
                }
            }
            false => None
        };
        Ok(OptimismBlockTxDetail::from(tx_ext, l1_tx_hash))
    }

    pub async fn find_tx_by_tx_batch_index_page_count(pool: web::Data<Pool>, tx_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTxPaging>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.inner_join(optimism_batches_ctc_txs::table.on(
                optimism_block_txs::index.eq(optimism_batches_ctc_txs::index).and(optimism_batches_ctc_txs::batch_index.eq(tx_batch_index.to_string()))
            ))
                .left_outer_join(optimism_tx_receipts::table.on(optimism_block_txs::columns::hash.eq(optimism_tx_receipts::columns::tx_hash)))
                .select((
                    optimism_block_txs::all_columns,
                    optimism_tx_receipts::columns::gas_used.nullable(),
                    optimism_tx_receipts::columns::contract_address.nullable()
                ))
                .order(optimism_block_txs_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }

    pub async fn find_tx_by_state_batch_index_page_count(pool: web::Data<Pool>, state_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTxPaging>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.inner_join(optimism_batches_scc_roots::table.on(
                optimism_block_txs::index.eq(optimism_batches_scc_roots::index).and(optimism_batches_scc_roots::batch_index.eq(state_batch_index.to_string()))
            )).left_outer_join(optimism_tx_receipts::table.on(optimism_block_txs::columns::hash.eq(optimism_tx_receipts::columns::tx_hash)))
                .select((
                    optimism_block_txs::all_columns,
                    optimism_tx_receipts::columns::gas_used.nullable(),
                    optimism_tx_receipts::columns::contract_address.nullable()
                ))
                .order(optimism_block_txs_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }

    pub async fn find_tx_count(conn: PgConn) -> Result<i64, ExpectedError> {
        let tx_count = web::block(move || {
            optimism_block_txs::table.count().first(&conn)
        }).await?;
        Ok(tx_count)
    }

    pub async fn find_tx_by_address_page_count(pool: web::Data<Pool>, address: String, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTxPaging>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.filter(from_address.eq(address.clone()).or(to_address.eq(address)))
                .left_outer_join(optimism_tx_receipts::table.on(optimism_block_txs::columns::hash.eq(optimism_tx_receipts::columns::tx_hash)))
                .select((
                    optimism_block_txs::all_columns,
                    optimism_tx_receipts::columns::gas_used.nullable(),
                    optimism_tx_receipts::columns::contract_address.nullable()
                ))
                .order(optimism_block_txs_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }

    pub async fn find_tx_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTxPaging>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table
                .left_outer_join(optimism_tx_receipts::table.on(optimism_block_txs::columns::hash.eq(optimism_tx_receipts::columns::tx_hash)))
                .select((
                    optimism_block_txs::all_columns,
                    optimism_tx_receipts::columns::gas_used.nullable(),
                    optimism_tx_receipts::columns::contract_address.nullable()
                ))
                .order(optimism_block_txs_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }
}

pub mod state_batch {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismStateBatch;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_batches_scc;
    use crate::schema::optimism::optimism_batches_scc::columns::*;

    pub async fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismStateBatch>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_batch = web::block(move || {
            optimism_batches_scc::table.into_boxed()
                .order(optimism_batches_scc_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_batch)
    }

    pub async fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismStateBatch, ExpectedError> {
        let conn = pool.get()?;
        let state_batch = web::block(move || {
            optimism_batches_scc::table.filter(batch_index.eq(index.to_string()))
                .first::<OptimismStateBatch>(&conn)
        }).await?;
        Ok(state_batch)
    }

    pub async fn find_latest_state_batch(conn: PgConn) -> Result<OptimismStateBatch, ExpectedError> {
        let latest_state_batch = web::block(move || {
            optimism_batches_scc::table.into_boxed()
                .order(optimism_batches_scc_id.desc())
                .first::<OptimismStateBatch>(&conn)
        }).await?;
        Ok(latest_state_batch)
    }
}

pub mod l1_to_l2 {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismL1ToL2Tx, OptimismL1ToL2TxSummary};
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::ethereum::ethereum_tx_logs;
    use crate::schema::optimism::optimism_block_txs;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_latest_l1_to_l2_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismL1ToL2TxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let l1_to_l2_tx_summary = web::block(move || {
            optimism_block_txs::table.inner_join(
                ethereum_tx_logs::table.on(optimism_block_txs::queue_index.eq(ethereum_tx_logs::queue_index))
            )
                .filter(optimism_block_txs::queue_origin.eq("l1"))
                .order(optimism_block_txs::optimism_block_txs_id.desc())
                .select((optimism_block_txs::l1_block_number, ethereum_tx_logs::tx_hash, optimism_block_txs::hash))
                .limit(10)
                .load::<OptimismL1ToL2TxSummary>(&conn)
        }).await?;
        Ok(l1_to_l2_tx_summary)
    }

    pub async fn find_l1_to_l2_tx_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismL1ToL2Tx>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_l1_to_l2_tx = web::block(move || {
            optimism_block_txs::table.inner_join(
                ethereum_tx_logs::table.on(optimism_block_txs::queue_index.eq(ethereum_tx_logs::queue_index))
            )
                .filter(optimism_block_txs::queue_origin.eq("l1"))
                .order(optimism_block_txs::optimism_block_txs_id.desc())
                .select((
                    optimism_block_txs::l1_block_number,
                    optimism_block_txs::queue_index,
                    optimism_block_txs::hash,
                    optimism_block_txs::l1_timestamp,
                    ethereum_tx_logs::tx_hash,
                    optimism_block_txs::l1_tx_origin,
                    optimism_block_txs::gas
                ))
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_l1_to_l2_tx)
    }
}

pub mod tx_logs {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismTxReceiptLog;
    use crate::schema::optimism::optimism_tx_receipt_logs;
    use crate::schema::optimism::optimism_tx_receipt_logs::columns::*;

    pub async fn find_tx_logs_by_hash(pool: web::Data<Pool>, hash: String) -> Result<Vec<OptimismTxReceiptLog>, ExpectedError> {
        let conn = pool.get()?;
        let tx_logs = web::block(move || {
            optimism_tx_receipt_logs::table.filter(tx_hash.eq(hash))
                .load::<OptimismTxReceiptLog>(&conn)
        }).await?;
        Ok(tx_logs)
    }
}

pub mod summary {
    use actix_web::web;
    use cached::proc_macro::cached;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::BoardSummary;
    use crate::repository::optimism::state_batch::find_latest_state_batch;
    use crate::repository::optimism::tx::find_tx_count;
    use crate::repository::optimism::tx_batch::find_latest_tx_batch;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_board_summary(pool: web::Data<Pool>) -> Result<BoardSummary, ExpectedError> {
        let conn = pool.get()?;
        let latest_tx_batch = find_latest_tx_batch(conn).await?;
        let conn = pool.get()?;
        let latest_state_batch = find_latest_state_batch(conn).await?;
        let conn = pool.get()?;
        let tx_count = find_tx_count(conn).await?;

        let board_summary = BoardSummary::new(
            latest_tx_batch.get_batch_index(),
            latest_state_batch.get_batch_index(),
            tx_count,
        );
        Ok(board_summary)
    }
}