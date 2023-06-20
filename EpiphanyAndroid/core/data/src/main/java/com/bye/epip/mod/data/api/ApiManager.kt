package com.bye.epip.mod.data.api

import com.bye.epip.lib.nethard.factory.RetrofitFactory
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class ApiManager @Inject constructor(
    private val retrofit: RetrofitFactory
) {


    fun getSoftApi(): SoftApi {
        return retrofit.create().create(SoftApi::class.java)
    }
}