package com.bye.epip.lib.middle.adapter

import android.content.Context
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.bye.epip.lib.middle.beans.SoftwareInfoBean
import com.bye.epip.lib.middle.databinding.MiddleHorizontalBigAdapterBinding

class HorizonBigAdapter: RecyclerView.Adapter<HorizonBigAdapter.ViewHolder>() {
    /**
     *
     */
    private val mDataResource=ArrayList<SoftwareInfoBean>()
    /**
     *
     */
    fun setNewData(ds:ArrayList<SoftwareInfoBean>){
        this.mDataResource.clear()
        this.mDataResource.addAll(ds)
        this.notifyDataSetChanged()
    }
    fun addNewData(d:SoftwareInfoBean){
        this.mDataResource.add(d)
        this.notifyDataSetChanged()
    }
    fun addNewData(ds:ArrayList<SoftwareInfoBean>){
        this.mDataResource.addAll(ds)
        this.notifyDataSetChanged()
    }

    /**
     *
     */
    override fun onBindViewHolder(holder: ViewHolder, position: Int) {

    }


    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val b = MiddleHorizontalBigAdapterBinding.inflate(
            LayoutInflater.from(parent.context),
            parent,
            false)
        return ViewHolder(b)
    }

    override fun getItemCount(): Int {
        return  mDataResource.size
    }

    class ViewHolder(private val B: MiddleHorizontalBigAdapterBinding) :RecyclerView.ViewHolder(B.root){

    }
}