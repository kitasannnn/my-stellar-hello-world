#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Symbol, Address};

// Định nghĩa key để lưu trữ tổng số tiền đã thanh toán trong storage
const TOTAL_PAID: Symbol = symbol_short!("T_PAID");

#[contract]
pub struct FreelanceContract;

#[contractimpl]
impl FreelanceContract {
    /// Hàm thực hiện thanh toán cho Freelancer
    /// env: Môi trường chạy contract
    /// from: Địa chỉ ví khách hàng (Client)
    /// to: Địa chỉ ví Freelancer
    /// amount: Số tiền thanh toán (Ví dụ: XLM hoặc USDC)
    pub fn pay_freelancer(env: Env, from: Address, to: Address, amount: i128) {
        // 1. Xác thực danh tính người gửi (Client) để đảm bảo an toàn
        from.require_auth();

        // 2. Logic chuyển tiền (Trong thực tế sẽ gọi đến Token Interface của Stellar)
        // Ở bước MVP này, chúng ta ghi nhận giao dịch lên storage
        let mut total: i128 = env.storage().instance().get(&TOTAL_PAID).unwrap_or(0);
        total += amount;
        
        // 3. Cập nhật dữ liệu lên Blockchain
        env.storage().instance().set(&TOTAL_PAID, &total);
    }

    /// Hàm kiểm tra tổng số tiền đã được thanh toán qua hệ thống
    pub fn get_total_paid(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_PAID).unwrap_or(0)
    }
    
    /// Hàm lấy lời chào định danh cho Freelancer (giống cái bro đã làm)
    pub fn get_status(env: Env) -> String {
        String::from_str(&env, "Freelance System Active")
    }
}