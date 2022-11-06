import http from "@/nethard/index";

export interface LevelTypeCategryReq {
      cate_type: number;
      level: number;
}

export interface CategryResult {
      cate_id: number;
      name: string;
      parent_id: number;
      level: number;
      child?: CategryResult[];
      cate_type?: number;
}

export const list_category = (params: LevelTypeCategryReq) => {
	return http.post<CategryResult[]>(`/manager/category/list_category`, params);
};
