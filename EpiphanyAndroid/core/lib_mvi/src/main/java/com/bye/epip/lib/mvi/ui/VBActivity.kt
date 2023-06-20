package com.bye.epip.lib.mvi.ui

import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.viewbinding.ViewBinding

abstract class VBActivity<VB:ViewBinding>:AppCompatActivity() {
    //
    lateinit var mBind: VB
    //
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(initVBAndGetRootView())
        setStatusBar()
    }



    abstract fun initVBAndGetRootView(): View

    open fun setStatusBar() {

    }
}