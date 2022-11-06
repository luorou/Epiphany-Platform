import http from "@/nethard/index";

export interface ReqLoginForm {
      account: string;
      password: string;
}

export interface LoginResult {
      access_token: string;
}


export const loginApi = (params: ReqLoginForm) => {
	return http.post<string>(`/manager/login/login_by_account`, params);
};