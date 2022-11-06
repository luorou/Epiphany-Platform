package com.bye.epip.mod.square.ui

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.square.databinding.SquareIndexFragmentBinding
import com.bye.epip.mod.square.databinding.SquareTodayGamesFragmentBinding

class TodayGamesFragment : VBFragment<SquareTodayGamesFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = SquareTodayGamesFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }
}