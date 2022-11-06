package com.bye.epip.mod.apps

import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.apps.databinding.AppsIndexFragmentBinding

class AppsIndexFragment :VBFragment<AppsIndexFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = AppsIndexFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }
}