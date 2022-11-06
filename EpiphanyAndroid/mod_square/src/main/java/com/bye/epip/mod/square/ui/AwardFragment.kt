package com.bye.epip.mod.square.ui

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.square.databinding.SquareAwardFragmentBinding
import com.bye.epip.mod.square.databinding.SquareIndexFragmentBinding

class AwardFragment : VBFragment<SquareAwardFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = SquareAwardFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }
}