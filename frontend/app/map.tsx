import { SafeAreaView } from 'react-native-safe-area-context';
import Navigation from './components/Navigation';
import MapPage from './components/MapPage/MapPage';

export default function Map() {
    return (
        <>
            <SafeAreaView className="flex-1 bg-[#242C38]">
                <MapPage />
            </SafeAreaView>
            <Navigation />
        </>
    );
}
