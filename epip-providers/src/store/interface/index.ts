import { MemberInfo} from "@/models/index"


export interface GlobalState {
	token: string|null;
	memberInfo: MemberInfo|null;
}