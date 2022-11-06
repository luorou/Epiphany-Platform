import http from "@/nethard/index";

export interface LoginReq {
      email: string;
      password: string;
}

export interface LoginResult {
      access_token: string;
}


export const loginApi = (params: LoginReq) => {
	return http.post<string>(`/provider/login/login_by_email`, params);
};