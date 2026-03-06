export const APP_NAME = "RestroSync";
export const APP_VERSION = "0.1.0";

export const API_VERSION = "v1";
export const API_BASE_URL = `/api/${API_VERSION}`;

export const DEFAULT_PAGE_SIZE = 20;
export const MAX_PAGE_SIZE = 100;

export const GST_RATES = {
  NONE: 0,
  GST_5: 5,
  GST_12: 12,
  GST_18: 18,
  GST_28: 28,
} as const;

export const CURRENCY = {
  code: "INR",
  symbol: "\u20B9",
  locale: "en-IN",
} as const;
