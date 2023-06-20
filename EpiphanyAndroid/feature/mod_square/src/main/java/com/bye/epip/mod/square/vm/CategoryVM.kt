package com.bye.epip.mod.square.vm

import androidx.lifecycle.viewModelScope
import com.blankj.utilcode.util.Utils
import com.bye.epip.lib.mvi.XViewModel
import com.bye.epip.lib.nethard.beans.ResponseX
import com.bye.epip.lib.nethard.utils.LocalJsonUtils
import com.bye.epip.mod.square.beans.CategoryBean
import com.bye.epip.mod.square.effect.CategoryEffect
import com.bye.epip.mod.square.effect.CategoryEvent
import com.bye.epip.mod.square.effect.CategoryState
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import kotlinx.coroutines.launch

class CategoryVM : XViewModel<CategoryState, CategoryEffect, CategoryEvent>() {

    override fun initState(): CategoryState {
        return CategoryState()
    }

    override fun process(ev: CategoryEvent) {
        if (ev is CategoryEvent.FetchGameCate) {
            fetchGameCate()
        }else if (ev is CategoryEvent.FetchAppsCate) {
            fetchAppsCate()
        }
    }



    /**
     *
     */
    private fun fetchGameCate() {
        viewModelScope.launch {
            val res = LocalJsonUtils.getLocalJson(Utils.getApp(),"mock/cate_games.json")
            val type = object : TypeToken<ResponseX<ArrayList<CategoryBean>?>>() {}.type
            val result  = Gson().fromJson<ResponseX<ArrayList<CategoryBean>?>>(res, type)
            //
            if (!result?.data.isNullOrEmpty()) {
                uiEffect = CategoryEffect.ReplyGamesCates(result.data!!)
            }
        }
    }

    /**
     *
     */
    private fun fetchAppsCate() {
        viewModelScope.launch {
            val res = LocalJsonUtils.getLocalJson(Utils.getApp(),"mock/cate_apps.json")
            val type = object : TypeToken<ResponseX<ArrayList<CategoryBean>?>>() {}.type
            val result  = Gson().fromJson<ResponseX<ArrayList<CategoryBean>?>>(res, type)
            //
            if (!result?.data.isNullOrEmpty()) {
                uiEffect = CategoryEffect.ReplyAppsCates(result.data!!)
            }
        }
    }

}