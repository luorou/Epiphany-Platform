import * as types from "@/store/mutation-types";

// * setToken
export const setToken = (token: string) => ({
	type: types.SET_TOKEN,
	token
});