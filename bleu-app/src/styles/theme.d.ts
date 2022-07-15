import { ButtonPropsVariantOverrides } from '@mui/material';

declare module '@mui/material/Button' {
  export interface ButtonPropsVariantOverrides {
    common: true;
  }
}

declare module '@mui/material/Typography' {
  export interface TypographyPropsVariantOverrides {
    mono: true;
  }
}
