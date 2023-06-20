package com.bye.epip.mod.games

import android.view.LayoutInflater
import android.view.View
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.games.databinding.GamesIndexFragmentBinding

class GamesIndexFragment :VBFragment<GamesIndexFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = GamesIndexFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }
}