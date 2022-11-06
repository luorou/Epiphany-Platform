
export interface MemberInfo {
      member_id: number;
      phone: string;
      email: string;
      header_url: string;
      status: number;
      organize_status: number;
}

export interface OrgnizeInfo {
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