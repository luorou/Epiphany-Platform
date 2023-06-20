package com.bye.epip.android.app

import android.app.Application
import com.didi.drouter.api.DRouter
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class EpiphanyApplication :Application(){
    override fun onCreate() {
        super.onCreate()
        DRouter.init(this)
    }
}