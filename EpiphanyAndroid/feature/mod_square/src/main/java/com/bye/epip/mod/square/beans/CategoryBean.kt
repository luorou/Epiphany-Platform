package com.bye.epip.mod.square.beans

data class CategoryBean(
    val cateId: Long,
    val name: String,
    val parentId: Long,
    val level: Long,
    val child: ArrayList<CategoryBean>? = null,
    val cateType: Long? = null
)
