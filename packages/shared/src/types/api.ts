export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  errors: ApiError[] | null;
  meta: PaginationMeta | null;
}

export interface ApiError {
  code: string;
  message: string;
  field: string | null;
}

export interface PaginationMeta {
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}

export interface HealthResponse {
  status: string;
  version: string;
  service: string;
}
