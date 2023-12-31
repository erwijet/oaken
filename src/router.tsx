import { createMemoryRouter, RouteObject } from 'react-router-dom';
import HomePage from '@/home/HomePage';
import TeamPage from './TeamPage';

const routes: RouteObject[] = [{
    path: '/',
    element: <HomePage />
}, {
    path: '/team/:id',
    element: <TeamPage />
}];

export const router = createMemoryRouter(routes, {
    initialEntries: ['/'],
    initialIndex: 1
});