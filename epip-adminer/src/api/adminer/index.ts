import http from "@/nethard/index";

export interface AdminerResult {
      id: number;
      admin_id: number;
      account: string;
      create_time: number;
      delete_time: number;
      status: number;
}

export const get_adaminers = () => {
	return http.post<AdminerResult[]>(`/manager/adminer/get_adaminers`);
};