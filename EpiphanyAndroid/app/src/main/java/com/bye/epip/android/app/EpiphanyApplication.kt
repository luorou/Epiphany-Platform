package com.bye.epip.android

import android.app.Application
import com.didi.drouter.api.DRouter

class EpiphanyApplication :Application(){
    override fun onCreate() {
        super.onCreate()
        DRouter.init(this)
    }
}