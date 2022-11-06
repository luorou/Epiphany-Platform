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

export const get_orgnize_list = () => {
      return http.post<OrgnizeResult[]>(`/provider/organize/get_orgnize_list`, {});
};