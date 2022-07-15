export function toEther(amount: string) {
  return +amount === 0 ? 0 : (+amount / Math.pow(10, 18)).toFixed(18);
}

export function txFee(gasUsed: string, gasPrice: string) {
  return +gasUsed === 0 || +gasPrice === 0 ? 0 : ((+gasUsed * +gasPrice) / Math.pow(10, 18)).toFixed(18);
}
