import http from "@/nethard/index";


export interface MemberInfoResult {
      member_id: number;
      phone: string;
      email: string;
      header_url: string;
      status: number;
      organize_status: number;
}


export const get_member_info = () => {
      return http.post<MemberInfoResult>(`/provider/member/get_member_info`, {});
};