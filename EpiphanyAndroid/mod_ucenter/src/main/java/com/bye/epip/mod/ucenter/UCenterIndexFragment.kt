package com.bye.epip.mod.ucenter

import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.ucenter.databinding.UcenterIndexFragmentBinding

class UCenterIndexFragment :VBFragment<UcenterIndexFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = UcenterIndexFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }
}