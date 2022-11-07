import { MemberInfo, OrgnizeInfo} from "@/models/index"


export interface GlobalState {
	token: string|null;
	memberInfo: MemberInfo | null;
	orgnizeInfo: OrgnizeInfo | null;
}