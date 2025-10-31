import { SafeAreaView } from 'react-native-safe-area-context';
import ShopPage from '../components/ShopPage/ShopPage';
import { Text } from 'react-native';
export default function Shop() {
    return (
        <>
            <SafeAreaView className="flex-1 bg-[#242C38]">
                <ShopPage />
            </SafeAreaView>
        </>
    );
}
