import { AnyAction } from "redux";
import produce from "immer";
import { GlobalState } from "@/store/interface";
import * as types from "@/store/mutation-types";
import { MemberInfo } from "@/models/index"

const globalState: GlobalState = {
	token: null,
	memberInfo: null,
	orgnizeInfo: null,
};

// global reducer
const global = (state: GlobalState = globalState, action: AnyAction) =>
	produce(state, draftState => {
		switch (action.type) {
			case types.SET_TOKEN:
				draftState.token = action.token;
				break;
			case types.SET_MEMBER_INFO:
				draftState.memberInfo = JSON.parse(action.info);
				break;
			case types.SET_ORGNIZE_INFO:
				draftState.orgnizeInfo = JSON.parse(action.info);
				break;

			default:
				return draftState;
		}
	});

export default global;
