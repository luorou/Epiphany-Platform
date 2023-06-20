package com.bye.epip.mod.square.effect

import com.bye.epip.mod.square.beans.CategoryBean

data class CategoryState(
    val count: String?=null
)

sealed class CategoryEvent{
    object FetchGameCate:CategoryEvent()
    object FetchAppsCate:CategoryEvent()
}

sealed class CategoryEffect {
    data class ReplyGamesCates(val result: ArrayList<CategoryBean>):CategoryEffect()
    data class ReplyAppsCates(val result: ArrayList<CategoryBean>):CategoryEffect()
}

