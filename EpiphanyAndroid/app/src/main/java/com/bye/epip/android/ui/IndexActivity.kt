package com.bye.epip.android

import android.os.Bundle
import android.view.View
import androidx.core.content.ContextCompat
import com.ashokvarma.bottomnavigation.BottomNavigationBar
import com.ashokvarma.bottomnavigation.BottomNavigationItem
import com.bye.epip.android.vp.IndexVpAdapter
import com.bye.epip.android.databinding.ActivityIndexBinding
import com.bye.epip.lib.mvi.ui.VBActivity
import com.didi.drouter.annotation.Router


@Router(path = "/page/index")
class IndexActivity : VBActivity<ActivityIndexBinding>() {

    override fun initVBAndGetRootView(): View {
        this.mBind = ActivityIndexBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        buildBottomBar()
    }

    private fun buildBottomBar() {
        mBind.vp2.isUserInputEnabled = false
        mBind.vp2.adapter = IndexVpAdapter(supportFragmentManager,lifecycle)
        //
        mBind.bottomBar.setMode(BottomNavigationBar.MODE_FIXED)
        mBind.bottomBar
            .addItem(
                BottomNavigationItem(
                    com.bye.epip.lib.resource.R.drawable.svg_icon_home,
                    resources.getString(com.bye.epip.lib.resource.R.string.text_square)
                ).setActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_d2691e))
                    .setInActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_999999))
            )
            .addItem(
                BottomNavigationItem(
                    com.bye.epip.lib.resource.R.drawable.svg_icon_community,
                    resources.getString(com.bye.epip.lib.resource.R.string.text_community)
                ).setActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_d2691e))
                    .setInActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_999999))
            )
            .addItem(
                BottomNavigationItem(
                    com.bye.epip.lib.resource.R.drawable.svg_icon_community,
                    resources.getString(com.bye.epip.lib.resource.R.string.text_community)
                ).setActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_d2691e))
                    .setInActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_999999))
            )
            .addItem(
                BottomNavigationItem(
                    com.bye.epip.lib.resource.R.drawable.svg_icon_mall,
                    resources.getString(com.bye.epip.lib.resource.R.string.text_mall)
                ).setActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_d2691e))
                    .setInActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_999999))
            )
            .addItem(
                BottomNavigationItem(
                    com.bye.epip.lib.resource.R.drawable.svg_icon_mime,
                    resources.getString(com.bye.epip.lib.resource.R.string.text_mine)
                ).setActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_d2691e))
                    .setInActiveColor(ContextCompat.getColor(this, com.bye.epip.lib.resource.R.color.col_999999))
            )
            .initialise()
        mBind.bottomBar.setTabSelectedListener(object :
            BottomNavigationBar.OnTabSelectedListener {
            override fun onTabSelected(position: Int) {
                mBind.vp2.currentItem = position
            }

            override fun onTabUnselected(position: Int) {
            }

            override fun onTabReselected(position: Int) {
            }
        })
    }

}