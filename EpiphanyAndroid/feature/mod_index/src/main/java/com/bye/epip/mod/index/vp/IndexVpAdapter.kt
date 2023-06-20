package com.bye.epip.mod.index.vp

import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.lifecycle.Lifecycle
import androidx.viewpager2.adapter.FragmentStateAdapter
import com.bye.epip.mod.apps.AppsIndexFragment
import com.bye.epip.mod.games.GamesIndexFragment
import com.bye.epip.mod.ranks.RanksIndexFragment
import com.bye.epip.mod.square.SquareIndexFragment
import com.bye.epip.mod.ucenter.UCenterIndexFragment


class IndexVpAdapter(fm: FragmentManager,lifecycle: Lifecycle) :
    FragmentStateAdapter(fm, lifecycle) {

    override fun getItemCount(): Int {
        return 5
    }

    override fun createFragment(position: Int): Fragment {
        return when (position) {
            0 -> {
                SquareIndexFragment()
            }
            1 -> {
                AppsIndexFragment()
            }
            2 -> {
                RanksIndexFragment()
            }
            3 -> {
                GamesIndexFragment()
            }
            4 -> {
                UCenterIndexFragment()
            }
            else -> Fragment()
        }
    }
}