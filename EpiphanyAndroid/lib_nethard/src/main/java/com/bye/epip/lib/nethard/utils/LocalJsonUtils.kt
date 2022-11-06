package com.bye.epip.lib.nethard.utils

import android.content.Context
import android.text.TextUtils
import com.google.gson.Gson
import java.io.BufferedReader
import java.io.IOException
import java.io.InputStreamReader
import java.lang.Exception

object LocalJsonUtils {
    /**
     *
     */
     suspend fun getLocalJson(context: Context, fileName: String?): String? {
        val builder = StringBuilder()
        val assetManager = context.assets
        try {
            val bufferedReader = BufferedReader(InputStreamReader(assetManager.open(fileName!!)))
            var line: String?
            while (bufferedReader.readLine().also { line = it } != null) {
                builder.append(line)
            }
        } catch (e: IOException) {
            e.printStackTrace()
            return null
        }
        return builder.toString()
    }

    suspend fun <T> localJson2Object(context: Context, fileName: String?, classOfT: Class<T>?): T? {
        val json = getLocalJson(context, fileName)
        if (TextUtils.isEmpty(json)) {
            return null
        }
        return try {
            Gson().fromJson(json, classOfT)
        } catch (e: Exception) {
//            Log.e("TAG----", "${e.message}")
            null
        }

    }

    fun bean2Json(bean: Any): String {
        return Gson().toJson(bean)
    }

    fun <T> json2bean(json: String?, classOfT: Class<T>?): T? {
        if (TextUtils.isEmpty(json)) {
            return null
        }
        return try {
            Gson().fromJson(json, classOfT)
        } catch (e: Exception) {
            null
        }

    }

}