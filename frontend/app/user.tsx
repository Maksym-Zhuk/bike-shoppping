import { View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import Navigation from './components/Navigation';
import UserPage from './components/UserPage/UserPage';

export default function User() {
    return (
        <View className="flex-1 bg-[#242C38]">
            <SafeAreaView className="flex-1 bg-[#242C38]">
                <UserPage />
            </SafeAreaView>
            <Navigation />
        </View>
    );
}
