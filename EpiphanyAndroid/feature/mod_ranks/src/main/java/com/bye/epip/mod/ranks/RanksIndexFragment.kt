package com.bye.epip.mod.ranks

import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.ranks.databinding.RanksIndexFragmentBinding

class RanksIndexFragment :VBFragment<RanksIndexFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = RanksIndexFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }
}