package com.bye.epip.mod.square.adapter

import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.lifecycle.Lifecycle
import androidx.viewpager2.adapter.FragmentStateAdapter
import com.bye.epip.mod.square.ui.*

class SquareIndexVpAdapter(fragmentManager: FragmentManager, lifecycle: Lifecycle) :
    FragmentStateAdapter(fragmentManager, lifecycle) {

    override fun getItemCount(): Int {
        return 5
    }

    override fun createFragment(position: Int): Fragment {
        return when (position) {
            0 -> {
                RecommendFragment()
            }
            1 -> {
                MustHaveFragment()
            }
            2 -> {
                CategoryFragment()
            }
            3 -> {
                AwardFragment()
            }
            4 -> {
                TodayGamesFragment()
            }
            else -> {
                Fragment()
            }
        }
    }

}