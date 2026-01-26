import streamlit as st
import pandas as pd
import numpy as np
import tensorflow as tf
import joblib
import plotly.express as px
from dataclasses import dataclass
from typing import Tuple, Optional

class Config:
    PAGE_TITLE = "Aimbot Hunter Engine"
    MODEL_PATH = 'aimbot_hunter_model.h5'
    SCALER_PATH = 'scaler.save'
    TIME_STEPS = 50
    CHEATER_THRESHOLD = 0.01 

@dataclass
class DetectionResult:
    """标准化的检测结果，类似于 Rust 的 Struct"""
    is_cheater: bool
    anomaly_score: float
    verdict_message: str
    processed_data: pd.DataFrame
    reconstruction_error: np.ndarray

class AimbotDetector:
    def __init__(self):
        self.model = self._load_model()
        self.scaler = self._load_scaler()

    @st.cache_resource
    def _load_model(_self):
        try:
            return tf.keras.models.load_model(Config.MODEL_PATH, compile=False)
        except Exception as e:
            st.error(f"❌ Failed to load model: {e}")
            return None

    @st.cache_resource
    def _load_scaler(_self):
        try:
            return joblib.load(Config.SCALER_PATH)
        except Exception:
            st.error(f"❌ Critical Error: Scaler file not found at {Config.SCALER_PATH}")
            return None

    def preprocess(self, df: pd.DataFrame) -> Tuple[np.ndarray, pd.DataFrame]:
        work_df = df.copy()
        work_df['dx'] = work_df['x'].diff().fillna(0)
        work_df['dy'] = work_df['y'].diff().fillna(0)
        
        raw_data = work_df[['dx', 'dy']].values
        scaled_data = self.scaler.transform(raw_data)
        
        X = []
        for i in range(len(scaled_data) - Config.TIME_STEPS):
            X.append(scaled_data[i : (i + Config.TIME_STEPS)])
            
        return np.array(X), work_df

    def analyze(self, df: pd.DataFrame) -> Optional[DetectionResult]:
        if self.model is None or self.scaler is None:
            return None
        X_input, processed_df = self.preprocess(df)
        
        if len(X_input) == 0:
            st.warning("⚠️ Not enough data to form an analysis window (need > 50 rows)")
            return None

        X_reconstructed = self.model.predict(X_input, verbose=0)
        
        mse_per_frame = np.mean(np.power(X_input - X_reconstructed, 2), axis=(1, 2))
        avg_mse = np.mean(mse_per_frame)
        
        is_cheater = avg_mse > Config.CHEATER_THRESHOLD
        
        return DetectionResult(
            is_cheater=is_cheater,
            anomaly_score=float(avg_mse),
            verdict_message="🚨 CHEATER DETECTED" if is_cheater else "✅ HUMAN PLAYER",
            processed_data=processed_df,
            reconstruction_error=mse_per_frame
        )

def render_dashboard():
    st.set_page_config(page_title=Config.PAGE_TITLE, layout="wide", page_icon="🛡️")
    
    st.title("🛡️ Aimbot Hunter")
    st.markdown("### Server-Side Anomaly Detection Engine (Prototype)")
    st.markdown("---")

    with st.sidebar:
        st.header("📂 Data Ingestion")
        uploaded_file = st.file_uploader("Upload Mouse Log (CSV)", type="csv")
        st.info("Supported Formats: Standard Mouse Logger CSV")

    if uploaded_file is not None:
        detector = AimbotDetector()
        raw_df = pd.read_csv(uploaded_file)
        
        with st.spinner('🤖 AI Brain is analyzing movement signatures...'):
            result = detector.analyze(raw_df)

        if result:
            render_verdict_section(result)
            
            render_analysis_section(result, raw_df)
    else:
        render_empty_state()

def render_verdict_section(res: DetectionResult):
    col1, col2, col3 = st.columns(3)
    
    with col1:
        st.metric(
            label="Anomaly Score (MSE)", 
            value=f"{res.anomaly_score:.5f}",
            delta="High Risk" if res.is_cheater else "Normal",
            delta_color="inverse"
        )
    
    with col2:
        st.metric(label="Confidence Threshold", value=Config.CHEATER_THRESHOLD)
        
    with col3:
        color = "red" if res.is_cheater else "green"
        st.markdown(f"""
            <div style="background-color:{color}; padding:10px; border-radius:5px; text-align:center;">
                <h3 style="color:white; margin:0;">{res.verdict_message}</h3>
            </div>
        """, unsafe_allow_html=True)

def render_analysis_section(res: DetectionResult, raw_df: pd.DataFrame):
    st.divider()
    st.subheader("📊 Forensics Analysis")
    
    tab1, tab2 = st.tabs(["Trajectory Replay", "Error Timeline"])
    
    with tab1:
        st.markdown("**Player Mouse Trajectory (2D Space)**")
        color_seq = ["#FF4B4B"] if res.is_cheater else ["#00CC96"]
        fig = px.scatter(
            raw_df, x='x', y='y', 
            color_discrete_sequence=color_seq,
            opacity=0.6
        )
        fig.update_yaxes(autorange="reversed")
        st.plotly_chart(fig, use_container_width=True)
        
    with tab2:
        st.markdown("**Reconstruction Error (Anomaly Spikes)**")
        fig_loss = px.line(
            y=res.reconstruction_error, 
            x=range(len(res.reconstruction_error)),
            labels={'x': 'Time Step (Frame)', 'y': 'MSE Loss'}
        )
        fig_loss.add_hline(
            y=Config.CHEATER_THRESHOLD, 
            line_dash="dash", line_color="red", 
            annotation_text="Detection Limit"
        )
        st.plotly_chart(fig_loss, use_container_width=True)

def render_empty_state():
    st.info("👈 Please upload a log file (`golden_human.csv` or `golden_bot.csv`) to start the inspection.")

if __name__ == "__main__":
    render_dashboard()