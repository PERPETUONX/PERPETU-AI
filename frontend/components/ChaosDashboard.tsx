import { useEffect, useState } from 'react';
import { ChaosMetric } from '../types';

export default function ChaosDashboard() {
    const [metrics, setMetrics] = useState<ChaosMetric>({
        freeWillIndex: 0,
        quantumEntanglement: 0,
        entropyLevel: 0
    });

    useEffect(() => {
        const ws = new WebSocket('wss://api.perpetu.ai/chaos-feed');
        
        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            setMetrics({
                freeWillIndex: calculateFreeWillIndex(data),
                quantumEntanglement: data.confidence * data.volume,
                entropyLevel: Math.log2(data.transactionCount / data.uniqueUsers)
            });
        };

        return () => ws.close();
    }, []);

    return (
        <div className="chaos-grid">
            <MetricDisplay 
                title="Free Will Index"
                value={metrics.freeWillIndex}
                uncertainty={0.05}
            />
            <WaveFunctionVisualizer 
                probabilities={metrics.quantumEntanglement}
            />
            <EntropyThermometer 
                value={metrics.entropyLevel}
            />
        </div>
    );
}
