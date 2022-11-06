package com.bye.epip.mod.square

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import com.angcyo.tablayout.delegate2.ViewPager2Delegate
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.square.adapter.SquareIndexVpAdapter
import com.bye.epip.mod.square.databinding.SquareIndexFragmentBinding

class SquareIndexFragment :VBFragment<SquareIndexFragmentBinding>(){
    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = SquareIndexFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        initView()
    }

    private fun initView() {
        val vpAdapter = SquareIndexVpAdapter(childFragmentManager, lifecycle)
        mBind.vp2.adapter = vpAdapter
        mBind.vp2.offscreenPageLimit = 5
        ViewPager2Delegate.install(mBind.vp2, mBind.tabLayout)
    }
}