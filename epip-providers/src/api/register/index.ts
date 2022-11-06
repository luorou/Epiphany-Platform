
import http from "@/nethard/index";


export interface RegisterReq {
      email: string;
      password: string;
      submitPassword: string;
}

export interface RegisterResult {
      member_id: number;
      phone: string;
      email: string;
      header_url: string;
      status: number;
      organize_status: number;
}


export const registerApi = (params: RegisterReq) => {
	return http.post<RegisterResult>(`/provider/register/register_by_email`, params);
};