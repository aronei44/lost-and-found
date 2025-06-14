import React, { useEffect, useRef, useState } from 'react';
import ModalComponent from './Modal';
import { apiClient } from '@/hooks/useApi';
import { useGlobalContext } from '@/hooks/globalprovider';
import Input from './Input';
import Card from './Card';
import Image from 'next/image';
import { serverKey } from '@/hooks/useServerKey';

const Camera = (p: {
    show: boolean;
    setShow: (show: boolean) => void;
}) => {
    const {
        state: {
            token
        }
    } = useGlobalContext();
    const videoRef = useRef<HTMLVideoElement>(null);
    const [isCameraOn, setIsCameraOn] = useState<boolean>(false);
    const [location, setLocation] = useState<{ latitude: number; longitude: number } | null>(null);
    const [recognized, setRecognized] = useState<Array<{
        bucket: string,
        recognized: Array<{
            id: number;
            fullname: string;
            alias: string;
            gender: string;
        }>,
        saved_file: string
    }>>([]);
    const [baseUrl, setBaseUrl] = useState<string>("");

    
    const getBaseUrl = async () => {
        const url = await serverKey("MINIO_URL");
        setBaseUrl(url);
    }


    const recognizeTarget = async (file: File) => {
        try {
            const formData = new FormData();
            formData.append('files', file);
            formData.append('latitude', location?.latitude.toString() ?? '');
            formData.append('longitude', location?.longitude.toString() ?? '');
            const res = await apiClient({
                method: "post",
                url: `/api/data/recognize_target`,
                headers: {
                    Authorization: `Bearer ${token.accessToken}`,
                    "Content-Type": "multipart/form-data"
                },
                data: formData
            })
            if (res.recognized?.length > 0) {
                setRecognized(prev => [res, ...prev]);
            }
        } catch (error) {
            console.error("Recognize Failed:", error);
        }
    }

    const handleCapture = () => {
        if (videoRef.current) {
            const canvas = document.createElement('canvas');
            canvas.width = videoRef.current.videoWidth;
            canvas.height = videoRef.current.videoHeight;
            const context = canvas.getContext('2d');
            if (context) {
                context.drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);
                canvas.toBlob(blob => {
                    if (blob) {
                        const file = new File([blob], 'captured-image.jpg', { type: 'image/jpeg' });
                        recognizeTarget(file);
                    }
                }, 'image/jpeg');
            }
        }
    }
    const autoCapture = () => {
        if (videoRef.current && isCameraOn && token.accessToken) {
            handleCapture();
            setTimeout(autoCapture, 1000);
        }
    }

    const handleLocation = () => {
        if (navigator.geolocation) {
            navigator.geolocation.getCurrentPosition(
                position => {
                    setLocation({
                        latitude: position.coords.latitude,
                        longitude: position.coords.longitude
                    });
                },
                error => {
                    console.error("Error getting location: ", error);
                }
            );
        }
    }

    useEffect(() => {
        if (isCameraOn && videoRef.current) {
            navigator.mediaDevices.getUserMedia({ video: true })
                .then(stream => {
                    if (videoRef.current) {
                        videoRef.current.srcObject = stream;
                        videoRef.current.play();
                    }
                })
                .catch(err => console.error("Error accessing camera: ", err));
        }
    }, [isCameraOn]);

    
    
    useEffect(() => {
        if (p.show) {
            setIsCameraOn(true);
            autoCapture();
            handleLocation();
            setRecognized([]);
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [p.show, isCameraOn, token.accessToken]);

    useEffect(() => {
        getBaseUrl();
    }, []);


    return (

        <ModalComponent
            show={p.show}
            onClose={() => {
                setIsCameraOn(false);
                p.setShow(false);
                if (videoRef.current) {
                    const stream = videoRef.current.srcObject as MediaStream;
                    if (stream) {
                        const tracks = stream.getTracks();
                        tracks.forEach(track => track.stop());
                    }
                    videoRef.current.srcObject = null;
                }
            }}
            title="Camera"
        >
            <div className="grid grid-cols-2">
                <div>
                    <video ref={videoRef} className="aspect-video" />
                    <div className="flex justify-center mt-2">
                        <Input
                            type='text'
                            label='Longitude'
                            value={`${location?.longitude ?? ''}`}
                            disabled
                        />
                        <Input
                            type='text'
                            label='Latitude'
                            value={`${location?.latitude ?? ''}`}
                            disabled
                        />
                    </div>
                </div>
                <div className='overflow-y-auto' style={{ maxHeight: '400px' }}>
                    {recognized.map((item, index) => (
                        <Card key={index + 1}>
                            <div className='flex gap-2'>
                                <Image alt={index + 'photo'} src={`${baseUrl}/${item.bucket}/${item.saved_file}`} width={100} height={100} />
                                <div>
                                    {item.recognized.map((recog, idx) => (
                                        <div key={idx + 'rec' + index} className='text-sm'>
                                            <span className='font-bold'>{recog.fullname}</span> ({recog.alias}) - {recog.gender}
                                        </div>
                                    ))}
                                </div>
                            </div>
                        </Card>
                    ))}
                </div>
            </div>
        </ModalComponent>
    );
}

export default Camera;