package com.bye.epip.mod.square.ui

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import androidx.fragment.app.viewModels
import androidx.recyclerview.widget.LinearLayoutManager
import com.bye.epip.lib.mvi.ui.MviFragment
import com.bye.epip.lib.mvi.ui.VBFragment
import com.bye.epip.mod.square.adapter.CateLevelOneAdapter
import com.bye.epip.mod.square.databinding.SquareCategoryFragmentBinding
import com.bye.epip.mod.square.databinding.SquareIndexFragmentBinding
import com.bye.epip.mod.square.effect.CategoryEffect
import com.bye.epip.mod.square.effect.CategoryEvent
import com.bye.epip.mod.square.effect.CategoryState
import com.bye.epip.mod.square.vm.CategoryVM

class CategoryFragment :MviFragment<CategoryState,
        CategoryEffect,
        CategoryEvent,
        CategoryVM,
        SquareCategoryFragmentBinding>(){

    override val VM: CategoryVM by viewModels()
    private val mGameAdapter = CateLevelOneAdapter()
    private val mAppsAdapter = CateLevelOneAdapter()

    override fun initVBAndGetRootView(layoutInflater: LayoutInflater): View {
        this.mBind = SquareCategoryFragmentBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        initView()
        initData()
    }


    private fun initView() {
        mBind.rvGames.layoutManager = LinearLayoutManager(requireContext())
        mBind.rvGames.adapter = mGameAdapter
        mBind.rvApps.layoutManager = LinearLayoutManager(requireContext())
        mBind.rvApps.adapter = mAppsAdapter
    }

    private fun initData() {
        VM.process(CategoryEvent.FetchGameCate)
        VM.process(CategoryEvent.FetchAppsCate)
    }


    override fun renderViewEffect(eff: CategoryEffect) {
        if (eff is CategoryEffect.ReplyGamesCates) {
            mGameAdapter.setDataS(eff.result)
        }else if (eff is CategoryEffect.ReplyAppsCates) {
            mAppsAdapter.setDataS(eff.result)
        }
    }
}