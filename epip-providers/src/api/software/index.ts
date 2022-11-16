
import { SoftwareInfo } from "@/models";
import http from "@/nethard/index";

export interface CreateSoftwareReq {
      app_name: string;
      app_logo: string;
      app_platform: number;
      app_pack_name: SVGStringList;
      privacy_protocal_url: string;
}

export const registerApi = (params: CreateSoftwareReq) => {
	return http.post<SoftwareInfo>(`/provider/register/register_by_email`, params);
};