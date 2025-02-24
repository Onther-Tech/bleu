import React from 'react';
import { Link, Typography } from '@mui/material';

function Logo() {
  return (
    <Typography variant="h5" sx={{ userSelect: 'none' }}>
      <Link href="/" color="inherit" underline="none">
        Tokamak Optimism Explorer
      </Link>
    </Typography>
  );
}

export default Logo;
