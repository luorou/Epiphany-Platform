package com.bye.epip.mod.square.adapter

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.bye.epip.mod.square.beans.CategoryBean
import com.bye.epip.mod.square.databinding.SquareCateLevelOneAdapterBinding
import com.bye.epip.mod.square.databinding.SquareCateLevelTwoAdapterBinding

class CateLeveltwoAdapter():RecyclerView.Adapter<CateLeveltwoAdapter.ViewHolder>() {

    private val mDataResource = ArrayList<CategoryBean>()


    fun setDataS(ds:ArrayList<CategoryBean>){
        this.mDataResource.addAll(ds)
        this.notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val b = SquareCateLevelTwoAdapterBinding.inflate(LayoutInflater.from(parent.context),parent,false)
        return ViewHolder(b)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        val item = mDataResource[position]
        holder.B.tvCateName.text = item.name
    }

    override fun getItemCount(): Int {
        return mDataResource.size
    }

    class ViewHolder(val B: SquareCateLevelTwoAdapterBinding) :RecyclerView.ViewHolder(B.root){}


}