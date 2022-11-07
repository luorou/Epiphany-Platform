import http from "@/nethard/index";

export interface OrgnizeResult {
      id: number;
      organize_id: number;
      member_id: number;
      name: string;
      phone: string;
      address: string;
      business_license_id: string;
      link_email: string;
      link_qq: string;
      organize_type: number;
      create_time: number;
      update_time: number;
      delete_time: number;
      status: number;
      logo: string;
      business_license_id_file: string;
}

export interface CreateOrgnizeReq {
      name: string;
      business_license_id: string;
      business_license_id_file: string;
      address: string;
      logo: string;
      phone: string;
      link_email: string;
      link_qq: string;
      organize_type: number;
}

export const get_organize_info = () => {
      return http.post<OrgnizeResult>(`/provider/organize/get_organize_info`, {});
};

export const create_orgnize = (params: CreateOrgnizeReq) => {
      return http.post<OrgnizeResult[]>(`/provider/organize/create_organize`, params);
};