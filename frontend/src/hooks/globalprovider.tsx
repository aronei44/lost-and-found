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
        </GlobalContext.Provider>
    );
}

export const useGlobalContext = (): typeof initialState => React.useContext(GlobalContext);
