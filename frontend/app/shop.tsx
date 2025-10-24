import { SafeAreaView } from 'react-native-safe-area-context';
import Navigation from './components/Navigation';
import ShopPage from './components/ShopPage/ShopPage';

export default function Shop() {
    return (
        <>
            <SafeAreaView className="flex-1 bg-[#242C38]">
                <ShopPage />
            </SafeAreaView>
            <Navigation />
        </>
    );
}
