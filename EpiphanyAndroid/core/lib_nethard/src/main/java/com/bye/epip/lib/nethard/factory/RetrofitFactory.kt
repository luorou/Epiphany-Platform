package com.bye.epip.lib.nethard.factory

import com.bye.epip.mod.global.env.EnvProvider
import okhttp3.Call
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class RetrofitFactory @Inject constructor(
    private val okhttpCallFactory: Call.Factory
) {

    @Singleton
    fun create(): Retrofit {
        return Retrofit.Builder()
            .baseUrl(EnvProvider.getBaseUrl())
            .callFactory(okhttpCallFactory)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
    }
}