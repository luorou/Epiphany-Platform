import * as types from "@/store/mutation-types";
import {MemberInfoResult } from "@/api/member/index"

// * setToken
export const setToken = (token: string) => ({
	type: types.SET_TOKEN,
	token
});

// * setToken
export const setMemberInfo = (info: string) => ({
	type: types.SET_MEMBER_INFO,
	info
});
//
export const setOrgnizeInfo = (info: string) => ({
	type: types.SET_ORGNIZE_INFO,
	info
});