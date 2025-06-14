'use client';
import React, { createContext, useEffect, useMemo, useState } from 'react';
import Main from '@/components/Main';
import Login from '@/components/Login';
import { apiClient } from './useApi';
import Swal from 'sweetalert2';
import Register from '@/components/Register';
import Profile from '@/components/Profile';
import LostPeopleAll from '@/components/LostPeopleAll';
import LostPeopleMonitored from '@/components/LostPeopleMonitored';
import PhotoButton from '@/components/PhotoButton';
import Camera from '@/components/Camera';
import { serverKey } from './useServerKey';
import ModalComponent from '@/components/Modal';

type GlobalState = {
    state: {
        activeState: string;
        activeComponent: React.ReactElement;
        authenticated: boolean;
        token: {
            accessToken: string;
            refreshToken: string;
        };
    },
    action: {
        setActiveState: (state: string) => void;
        setAuthenticated: (authenticated: boolean) => void;
        setToken: (token: { accessToken: string; refreshToken: string }) => void;
        setLogout: () => void;
    }
}

const initialState: GlobalState = {
    state: {
        activeState: 'main',
        activeComponent: <></>,
        authenticated: false,
        token: {
            accessToken: '',
            refreshToken: ''
        }
    },
    action: {
        setActiveState: (state: string) => {
            initialState.state.activeState = state;
        },
        setAuthenticated: (authenticated: boolean) => {
            initialState.state.authenticated = authenticated;
        },
        setToken: (token: { accessToken: string; refreshToken: string }) => {
            initialState.state.token = token;
        },
        setLogout: () => {
            initialState.state.activeState = 'main';
            initialState.state.authenticated = false;
            initialState.state.token = {
                accessToken: '',
                refreshToken: ''
            };
        }
    }
};

// context and initial state
const GlobalContext = createContext<GlobalState>(initialState);

export function GlobalProvider(p: Readonly<React.PropsWithChildren>) {

    const [activeState, setActiveState] = useState<string>("main");
    const [activeComponent, setActiveComponent] = useState<React.ReactElement>(<></>);
    const [authenticated, setAuthenticated] = useState<boolean>(false);
    const [token, setToken] = useState<{
        accessToken: string;
        refreshToken: string;
    }>({
        accessToken: '',
        refreshToken: ''
    })
    const [cameraModal, setCameraModal] = useState<boolean>(false);
    const [messageModal, setMessageModal] = useState<boolean>(false);
    const [messages, setMessages] = useState<Array<{
        alias: string,
        fullname: string,
        is_read: boolean,
        date: Date
    }>>([]);

    const getComponent = (state: string) => {
        const components : Record<string, React.ReactElement> = {
            main: <Main />,
            login: <Login />,
            register: <Register />,
            profile: <Profile />,
            lost_people: <LostPeopleAll />,
            lost_people_monitored: <LostPeopleMonitored />
        }
        setActiveComponent(components[state] || <></>);
    }

    useEffect(() => {
        getComponent(activeState);
    }, [activeState]);


    const setLogout = () => {
        setActiveState("main");
        setAuthenticated(false);
        setToken({
            accessToken: '',
            refreshToken: ''
        });
        localStorage.removeItem("accessToken");
        localStorage.removeItem("refreshToken");
        Swal.fire({
            title: "Logout",
            text: "Sesion has been terminated.",
            icon: "success",
            confirmButtonText: "OK"
        });
    }

    const revalidateToken = async (tkn: string) => {
        try {
            const response = await apiClient({
                method: "post",
                url: "/api/auth/refresh",
                data: {
                    refresh_token: tkn
                }
            })
            setToken({
                accessToken: response.access_token,
                refreshToken: response.refresh_token
            });
            setAuthenticated(true);
        } catch (error) {
            console.error("Token revalidation failed:", error);
            setLogout();
        }
    }

    useEffect(() => {
        if (authenticated && token.refreshToken) {
            localStorage.setItem("accessToken", token.accessToken);
            localStorage.setItem("refreshToken", token.refreshToken);
            const interval = setInterval(() => {
                revalidateToken(token.refreshToken);
            }, 10 * 60 * 1000); // Revalidate every 10 minutes

            return () => clearInterval(interval);
        }

    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [token]);

    useEffect(() => {
        const storedAccessToken = localStorage.getItem("accessToken");
        const storedRefreshToken = localStorage.getItem("refreshToken");
        if (storedAccessToken && storedRefreshToken) {
            setToken({
                accessToken: storedAccessToken,
                refreshToken: storedRefreshToken
            });
            revalidateToken(storedRefreshToken);
        }
        
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);


    const streaming = async () => {
        try {
            let baseUrl = await serverKey("API_BASE_URL");
            baseUrl = baseUrl.replace("http://", "ws://").replace("https://", "wss://");
            const url = `${baseUrl}/api/stream?token=${token.accessToken}`;
            const socket = new WebSocket(url);
            socket.onopen = () => {
                console.log("WebSocket connection established");
            }
            socket.onmessage = (event) => {
                const data = JSON.parse(event.data);
                if (data.type === "recognition") {
                    const person = data.data ?? {};
                    if (person.alias && person.fullname) {
                        setMessages(prevMessages => [{
                            alias: person.alias,
                            fullname: person.fullname,
                            is_read: false,
                            date: new Date()
                        }, ...prevMessages]);
                    }
                }
            }
            socket.onclose = () => {
                console.log("WebSocket closed, retrying in 1s");
                setTimeout(streaming, 1000);
            }

            socket.onerror = (err) => {
                console.error("WebSocket error", err);
                socket.close();
            }
        } catch (error) {
            console.error("Error in streaming:", error);
        }
    }

    useEffect(() => {
        if (authenticated && token.accessToken) {
            streaming();
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [authenticated, token.accessToken]);


    const context = useMemo(() => {
        return {
            state: {
                activeState,
                activeComponent,
                authenticated,
                token
            },
            action: {
                setActiveState,
                setAuthenticated,
                setToken,
                setLogout
            }
        }
    }, [activeState, activeComponent, authenticated, token]);

    return (
        <GlobalContext.Provider
            value={context}
        >
            {p.children}
            {authenticated && (
                <>
                    <PhotoButton
                        onClick={() => setCameraModal(true)}
                    >
                        <div className='text-2xl'>+</div>
                    </PhotoButton>
                    <PhotoButton
                        onClick={() => setMessageModal(true)}
                        scale_right={2}
                    >
                        <div className='text-xl'>{messages.filter(m => !m.is_read).length}</div>
                    </PhotoButton>
                </>
            )}
            <Camera
                show={cameraModal}
                setShow={setCameraModal}
            />
            <ModalComponent
                show={messageModal}
                onClose={() => {
                    setMessageModal(false);
                    setMessages(prevMessages => prevMessages.map(m => ({ ...m, is_read: true })));
                }}
                title="Messages"
            >
                <div className="max-h-[80vh] overflow-y-auto">
                    {messages.map((message, index) => (
                        <div key={index + 1} className={`p-4 ${message.is_read ? 'bg-gray-100' : 'bg-blue-100'} mb-4`}>
                            <p>{message.fullname} ({message.alias}) ditemukan !!!</p>
                            <span className="text-sm text-gray-500">{new Date(message.date).toLocaleString()}</span>
                        </div>
                    ))}
                </div>
            </ModalComponent>
        </GlobalContext.Provider>
    );
}

export const useGlobalContext = (): typeof initialState => React.useContext(GlobalContext);
