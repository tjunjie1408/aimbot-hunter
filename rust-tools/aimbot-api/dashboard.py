import streamlit as st
import requests
import pandas as pd
import numpy as np
import time
import math
import random

API_URL = "http://127.0.0.1:3002/analyze"

st.set_page_config(page_title="Aimbot Hunter Dashboard", layout="wide")

st.title("🛡️ Aimbot Hunter Command Center")
st.markdown("### Powered by Rust 🦀 & ONNX Runtime 🧠")

with st.sidebar:
    st.header("🎮 Simulation Controls")
    sim_type = st.radio("Select Behavior:", ["Human (Smooth)", "Aimbot (Snap/Teleport)"])
    
    threshold = st.slider("Anomaly Threshold (Sync with Rust config)", 0.0, 1.0, 0.25)
    
    st.info("Click the button below to generate trajectory and send to Rust API.")
    run_btn = st.button("🚀 Analyze Trajectory", type="primary")

def generate_human_path():
    path = []
    for i in range(60):
        x = i * 5.0
        y = math.sin(i * 0.15) * 30.0 + random.uniform(-2, 2)
        path.append({"x": x, "y": y})
    return path

def generate_aimbot_path():
    path = []
    x, y = 0, 0
    for i in range(60):
        path.append({"x": float(x), "y": float(y)})
        if i % 15 == 0:
            x += 150
            y += 100
        else:
            x += random.uniform(-0.5, 0.5)
            y += random.uniform(-0.5, 0.5)
    return path

col1, col2 = st.columns([2, 1])

if run_btn:
    if "Human" in sim_type:
        data = generate_human_path()
        label = "Human-like Movement"
    else:
        data = generate_aimbot_path()
        label = "Aimbot-like Movement"

    payload = {"trajectory": data}
    
    try:
        start_time = time.time()
        response = requests.post(API_URL, json=payload)
        end_time = time.time()
        latency_ms = (end_time - start_time) * 1000
        
        result = response.json()
        score = result['anomaly_score']

        with col1:
            st.subheader(f"📍 Trajectory Visualizer: {label}")

            df = pd.DataFrame(data)
            df['step'] = range(len(df))

            import altair as alt

            chart = alt.Chart(df).mark_line(point=True, strokeWidth=3).encode(
                x=alt.X('x', title='Screen X Position', scale=alt.Scale(zero=False)),
                y=alt.Y('y', title='Screen Y Position', scale=alt.Scale(zero=False)),
                order='step',
                tooltip=['step', 'x', 'y']
            ).properties(
                height=400
            ).interactive()

            st.altair_chart(chart, use_container_width=True)
            st.caption("Actual mouse path on screen (X vs Y)")

        with col2:
            st.subheader("🧠 AI Verdict")

            st.metric("Rust Engine Latency", f"{latency_ms:.2f} ms", delta_color="inverse")

            st.metric("Anomaly Score (MSE)", f"{score:.6f}")

            st.markdown("---")
            if score > threshold:
                st.error(f"## 🚨 CHEATER DETECTED")
                st.write(f"Score ({score:.4f}) > Threshold ({threshold})")
            else:
                st.success(f"## ✅ HUMAN VERIFIED")
                st.write(f"Score ({score:.4f}) <= Threshold ({threshold})")
                
            st.json(result)

    except Exception as e:
        st.error(f"Connection Failed: {e}")
        st.warning("Make sure your Rust backend is running! (`cargo run --release`)")

else:
    with col1:
        st.info("👈 Use the sidebar to start a simulation.")