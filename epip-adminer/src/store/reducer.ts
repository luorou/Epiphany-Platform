import { legacy_createStore as createStore, combineReducers, Store, compose } from "redux";
import global from "./modules/global/reducer";


// 创建reducer(拆分reducer)
const reducer = combineReducers({
	global,
});
export default reducer;