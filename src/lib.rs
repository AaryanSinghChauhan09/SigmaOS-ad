//! SigmaOS-ad — comprehensive Active Directory component.
//!
//! Provides Active Directory functionality for identity management,
//! including users, groups, organizational units, and domain services.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a simple GUID
fn generate_guid() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let random_part = (timestamp % 0xFFFFFFFF) as u32;
    format!("{{{:08x}-{:04x}-{:04x}-{:04x}-{:012x}}}", 
        random_part, 
        (timestamp >> 32) as u16 & 0xFFFF,
        (timestamp >> 48) as u16 & 0xFFFF,
        (timestamp >> 64) as u16 & 0xFFFF,
        timestamp & 0xFFFFFFFFFFFF)
}

/// Active Directory object types
#[derive(Debug, Clone, PartialEq)]
pub enum ADObjectType {
    /// Domain
    Domain,
    /// Organizational Unit
    OrganizationalUnit,
    /// User
    User,
    /// Group
    Group,
    /// Computer
    Computer,
    /// Contact
    Contact,
    /// Printer
    Printer,
    /// Shared Folder
    SharedFolder,
}

/// Active Directory object
#[derive(Debug, Clone)]
pub struct ADObject {
    /// Object GUID
    pub guid: String,
    /// Object type
    pub object_type: ADObjectType,
    /// Common name
    pub cn: String,
    /// Distinguished name
    pub dn: String,
    /// Description
    pub description: String,
    /// Created timestamp
    pub created_at: u64,
    /// Modified timestamp
    pub modified_at: u64,
    /// Attributes
    pub attributes: HashMap<String, String>,
    /// Parent DN
    pub parent_dn: String,
    /// Child objects
    pub children: Vec<String>,
}

/// Active Directory user
#[derive(Debug, Clone)]
pub struct ADUser {
    /// Base object
    pub object: ADObject,
    /// Username
    pub username: String,
    /// Display name
    pub display_name: String,
    /// Email
    pub email: String,
    /// Department
    pub department: String,
    /// Title
    pub title: String,
    /// Manager
    pub manager: String,
    /// Account enabled
    pub account_enabled: bool,
    /// Password last set
    pub password_last_set: u64,
    /// Groups
    pub groups: Vec<String>,
}

/// Active Directory group
#[derive(Debug, Clone)]
pub struct ADGroup {
    /// Base object
    pub object: ADObject,
    /// Group name
    pub group_name: String,
    /// Group type
    pub group_type: GroupType,
    /// Group scope
    pub group_scope: GroupScope,
    /// Members
    pub members: Vec<String>,
    /// Nested groups
    pub nested_groups: Vec<String>,
}

/// Group type
#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    /// Security group
    Security,
    /// Distribution group
    Distribution,
    /// Both security and distribution
    Both,
}

/// Group scope
#[derive(Debug, Clone, PartialEq)]
pub enum GroupScope {
    /// Domain local
    DomainLocal,
    /// Global
    Global,
    /// Universal
    Universal,
}

/// Active Directory domain
#[derive(Debug, Clone)]
pub struct ADDomain {
    /// Domain name
    pub domain_name: String,
    /// NetBIOS name
    pub netbios_name: String,
    /// Domain SID
    pub domain_sid: String,
    /// Domain controllers
    pub domain_controllers: Vec<String>,
    /// Sites
    pub sites: Vec<String>,
    /// Subnets
    pub subnets: Vec<Subnet>,
    /// Forest
    pub forest: String,
    /// Domain mode
    pub domain_mode: DomainMode,
}

/// Subnet
#[derive(Debug, Clone)]
pub struct Subnet {
    /// Subnet name
    pub name: String,
    /// Subnet prefix
    pub prefix: String,
    /// Site
    pub site: String,
}

/// Domain mode
#[derive(Debug, Clone, PartialEq)]
pub enum DomainMode {
    /// Windows 2000 mixed mode
    Mixed2000,
    /// Windows 2000 native mode
    Native2000,
    /// Windows 2003 mode
    Windows2003,
    /// Windows 2008 mode
    Windows2008,
    /// Windows 2012 mode
    Windows2012,
    /// Windows 2016 mode
    Windows2016,
    /// Windows 2019 mode
    Windows2019,
    /// Windows 2022 mode
    Windows2022,
}

/// Active Directory service
#[derive(Debug, Clone)]
pub struct ADService {
    /// Domain
    pub domain: ADDomain,
    /// Objects
    pub objects: HashMap<String, ADObject>,
    /// Users
    pub users: HashMap<String, ADUser>,
    /// Groups
    pub groups: HashMap<String, ADGroup>,
    /// OUs
    pub organizational_units: HashMap<String, ADObject>,
    /// Computers
    pub computers: HashMap<String, ADObject>,
    /// Statistics
    pub stats: ADStatistics,
}

/// Active Directory statistics
#[derive(Debug, Clone, Default)]
pub struct ADStatistics {
    /// Total objects
    pub total_objects: u64,
    /// Total users
    pub total_users: u64,
    /// Total groups
    pub total_groups: u64,
    /// Total OUs
    pub total_ous: u64,
    /// Total computers
    pub total_computers: u64,
    /// Total contacts
    pub total_contacts: u64,
    /// Total printers
    pub total_printers: u64,
    /// Last backup
    pub last_backup: u64,
}

impl Default for ADService {
    fn default() -> Self {
        Self {
            domain: ADDomain {
                domain_name: "sigmaos.local".into(),
                netbios_name: "SIGMAOS".into(),
                domain_sid: "S-1-5-21-1234567890-1234567890-1234567890".into(),
                domain_controllers: vec!["dc1.sigmaos.local".into()],
                sites: vec!["Default-First-Site-Name".into()],
                subnets: vec![],
                forest: "sigmaos.local".into(),
                domain_mode: DomainMode::Windows2022,
            },
            objects: HashMap::new(),
            users: HashMap::new(),
            groups: HashMap::new(),
            organizational_units: HashMap::new(),
            computers: HashMap::new(),
            stats: ADStatistics::default(),
        }
    }
}

impl ADService {
    /// Create new AD service
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default objects
    pub fn initialize(&mut self) {
        // Create default OUs
        self.create_organizational_unit("OU=Users,DC=sigmaos,DC=local".into(), "Users".into());
        self.create_organizational_unit("OU=Computers,DC=sigmaos,DC=local".into(), "Computers".into());
        self.create_organizational_unit("OU=Groups,DC=sigmaos,DC=local".into(), "Groups".into());
        
        // Create default groups
        self.create_group("Domain Admins".into(), GroupType::Security, GroupScope::Global);
        self.create_group("Domain Users".into(), GroupType::Security, GroupScope::Global);
        self.create_group("Domain Computers".into(), GroupType::Security, GroupScope::Global);
        
        // Create default admin user
        self.create_user("Administrator".into(), "Administrator".into(), "admin@sigmaos.local".into());
    }
    
    /// Create organizational unit
    pub fn create_organizational_unit(&mut self, dn: String, name: String) -> String {
        let object = ADObject {
            guid: format!("{{{}}}", generate_guid()),
            object_type: ADObjectType::OrganizationalUnit,
            cn: name.clone(),
            dn: dn.clone(),
            description: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attributes: HashMap::new(),
            parent_dn: "DC=sigmaos,DC=local".into(),
            children: vec![],
        };
        
        self.organizational_units.insert(dn.clone(), object.clone());
        self.objects.insert(dn.clone(), object);
        self.stats.total_ous += 1;
        self.stats.total_objects += 1;
        
        dn
    }
    
    /// Create user
    pub fn create_user(&mut self, username: String, display_name: String, email: String) -> String {
        let dn = format!("CN={},OU=Users,DC=sigmaos,DC=local", username);
        
        let mut attributes = HashMap::new();
        attributes.insert("sAMAccountName".into(), username.clone());
        attributes.insert("userPrincipalName".into(), format!("{}@sigmaos.local", username));
        attributes.insert("mail".into(), email.clone());
        attributes.insert("displayName".into(), display_name.clone());
        
        let object = ADObject {
            guid: format!("{{{}}}", generate_guid()),
            object_type: ADObjectType::User,
            cn: username.clone(),
            dn: dn.clone(),
            description: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attributes,
            parent_dn: "OU=Users,DC=sigmaos,DC=local".into(),
            children: vec![],
        };
        
        let user = ADUser {
            object: object.clone(),
            username,
            display_name,
            email,
            department: String::new(),
            title: String::new(),
            manager: String::new(),
            account_enabled: true,
            password_last_set: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            groups: vec![],
        };
        
        self.users.insert(dn.clone(), user);
        self.objects.insert(dn.clone(), object);
        self.stats.total_users += 1;
        self.stats.total_objects += 1;
        
        dn
    }
    
    /// Create group
    pub fn create_group(&mut self, group_name: String, group_type: GroupType, group_scope: GroupScope) -> String {
        let dn = format!("CN={},OU=Groups,DC=sigmaos,DC=local", group_name);
        
        let object = ADObject {
            guid: format!("{{{}}}", generate_guid()),
            object_type: ADObjectType::Group,
            cn: group_name.clone(),
            dn: dn.clone(),
            description: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attributes: HashMap::new(),
            parent_dn: "OU=Groups,DC=sigmaos,DC=local".into(),
            children: vec![],
        };
        
        let group = ADGroup {
            object: object.clone(),
            group_name,
            group_type,
            group_scope,
            members: vec![],
            nested_groups: vec![],
        };
        
        self.groups.insert(dn.clone(), group);
        self.objects.insert(dn.clone(), object);
        self.stats.total_groups += 1;
        self.stats.total_objects += 1;
        
        dn
    }
    
    /// Create computer
    pub fn create_computer(&mut self, computer_name: String) -> String {
        let dn = format!("CN={},OU=Computers,DC=sigmaos,DC=local", computer_name);
        
        let object = ADObject {
            guid: format!("{{{}}}", generate_guid()),
            object_type: ADObjectType::Computer,
            cn: computer_name.clone(),
            dn: dn.clone(),
            description: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attributes: HashMap::new(),
            parent_dn: "OU=Computers,DC=sigmaos,DC=local".into(),
            children: vec![],
        };
        
        self.computers.insert(dn.clone(), object.clone());
        self.objects.insert(dn.clone(), object);
        self.stats.total_computers += 1;
        self.stats.total_objects += 1;
        
        dn
    }
    
    /// Get user by username
    pub fn get_user_by_username(&self, username: &str) -> Option<&ADUser> {
        self.users.values()
            .find(|u| u.username == username)
    }
    
    /// Get user by DN
    pub fn get_user_by_dn(&self, dn: &str) -> Option<&ADUser> {
        self.users.get(dn)
    }
    
    /// Get group by name
    pub fn get_group_by_name(&self, group_name: &str) -> Option<&ADGroup> {
        self.groups.values()
            .find(|g| g.group_name == group_name)
    }
    
    /// Get group by DN
    pub fn get_group_by_dn(&self, dn: &str) -> Option<&ADGroup> {
        self.groups.get(dn)
    }
    
    /// Get object by DN
    pub fn get_object(&self, dn: &str) -> Option<&ADObject> {
        self.objects.get(dn)
    }
    
    /// Get all objects
    pub fn get_all_objects(&self) -> Vec<&ADObject> {
        self.objects.values().collect()
    }
    
    /// Get objects by type
    pub fn get_objects_by_type(&self, object_type: &ADObjectType) -> Vec<&ADObject> {
        self.objects.values()
            .filter(|o| o.object_type == *object_type)
            .collect()
    }
    
    /// Add user to group
    pub fn add_user_to_group(&mut self, user_dn: &str, group_dn: &str) -> bool {
        // Add user to group members
        if let Some(group) = self.groups.get_mut(group_dn) {
            if !group.members.contains(&user_dn.to_string()) {
                group.members.push(user_dn.to_string());
            }
        }
        
        // Add group to user groups
        if let Some(user) = self.users.get_mut(user_dn) {
            if !user.groups.contains(&group_dn.to_string()) {
                user.groups.push(group_dn.to_string());
            }
        }
        
        true
    }
    
    /// Remove user from group
    pub fn remove_user_from_group(&mut self, user_dn: &str, group_dn: &str) -> bool {
        // Remove user from group members
        if let Some(group) = self.groups.get_mut(group_dn) {
            if let Some(pos) = group.members.iter().position(|m| m == user_dn) {
                group.members.remove(pos);
            }
        }
        
        // Remove group from user groups
        if let Some(user) = self.users.get_mut(user_dn) {
            if let Some(pos) = user.groups.iter().position(|g| g == group_dn) {
                user.groups.remove(pos);
            }
        }
        
        true
    }
    
    /// Get user's groups
    pub fn get_user_groups(&self, user_dn: &str) -> Vec<String> {
        self.users.get(user_dn)
            .map(|u| u.groups.clone())
            .unwrap_or_default()
    }
    
    /// Get group's members
    pub fn get_group_members(&self, group_dn: &str) -> Vec<String> {
        self.groups.get(group_dn)
            .map(|g| g.members.clone())
            .unwrap_or_default()
    }
    
    /// Check if user is member of group
    pub fn is_user_in_group(&self, user_dn: &str, group_dn: &str) -> bool {
        self.users.get(user_dn)
            .map(|u| u.groups.contains(&group_dn.to_string()))
            .unwrap_or(false)
    }
    
    /// Search users
    pub fn search_users(&self, query: &str) -> Vec<&ADUser> {
        let query_lower = query.to_lowercase();
        self.users.values()
            .filter(|u| {
                u.username.to_lowercase().contains(&query_lower) ||
                u.display_name.to_lowercase().contains(&query_lower) ||
                u.email.to_lowercase().contains(&query_lower) ||
                u.department.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Search groups
    pub fn search_groups(&self, query: &str) -> Vec<&ADGroup> {
        let query_lower = query.to_lowercase();
        self.groups.values()
            .filter(|g| g.group_name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get AD statistics
    pub fn get_statistics(&self) -> &ADStatistics {
        &self.stats
    }
    
    /// Export AD data
    pub fn export_data(&self) -> ADExport {
        ADExport {
            domain: self.domain.clone(),
            users: self.users.values().cloned().collect(),
            groups: self.groups.values().cloned().collect(),
            organizational_units: self.organizational_units.values().cloned().collect(),
            computers: self.computers.values().cloned().collect(),
        }
    }
    
    /// Import AD data
    pub fn import_data(&mut self, export: ADExport) {
        self.domain = export.domain;
        
        for user in export.users {
            self.users.insert(user.object.dn.clone(), user);
        }
        
        for group in export.groups {
            self.groups.insert(group.object.dn.clone(), group);
        }
        
        for ou in export.organizational_units {
            self.organizational_units.insert(ou.dn.clone(), ou);
        }
        
        for computer in export.computers {
            self.computers.insert(computer.dn.clone(), computer);
        }
        
        // Recalculate statistics
        self.stats.total_users = self.users.len() as u64;
        self.stats.total_groups = self.groups.len() as u64;
        self.stats.total_ous = self.organizational_units.len() as u64;
        self.stats.total_computers = self.computers.len() as u64;
        self.stats.total_objects = self.objects.len() as u64;
    }
}

/// AD export data
#[derive(Debug, Clone)]
pub struct ADExport {
    /// Domain
    pub domain: ADDomain,
    /// Users
    pub users: Vec<ADUser>,
    /// Groups
    pub groups: Vec<ADGroup>,
    /// Organizational units
    pub organizational_units: Vec<ADObject>,
    /// Computers
    pub computers: Vec<ADObject>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ad_object_type() {
        let types = vec![
            ADObjectType::Domain,
            ADObjectType::OrganizationalUnit,
            ADObjectType::User,
            ADObjectType::Group,
            ADObjectType::Computer,
            ADObjectType::Contact,
            ADObjectType::Printer,
            ADObjectType::SharedFolder,
        ];
        assert_eq!(types.len(), 8);
    }
    
    #[test]
    fn test_group_type() {
        let types = vec![
            GroupType::Security,
            GroupType::Distribution,
            GroupType::Both,
        ];
        assert_eq!(types.len(), 3);
    }
    
    #[test]
    fn test_group_scope() {
        let scopes = vec![
            GroupScope::DomainLocal,
            GroupScope::Global,
            GroupScope::Universal,
        ];
        assert_eq!(scopes.len(), 3);
    }
    
    #[test]
    fn test_domain_mode() {
        let modes = vec![
            DomainMode::Mixed2000,
            DomainMode::Native2000,
            DomainMode::Windows2003,
            DomainMode::Windows2008,
            DomainMode::Windows2012,
            DomainMode::Windows2016,
            DomainMode::Windows2019,
            DomainMode::Windows2022,
        ];
        assert_eq!(modes.len(), 8);
    }
    
    #[test]
    fn test_service_creation() {
        let service = ADService::new();
        assert_eq!(service.get_statistics().total_users, 0);
        assert_eq!(service.get_statistics().total_groups, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut service = ADService::new();
        service.initialize();
        
        assert!(service.get_statistics().total_ous > 0);
        assert!(service.get_statistics().total_groups > 0);
        assert!(service.get_statistics().total_users > 0);
    }
    
    #[test]
    fn test_create_organizational_unit() {
        let mut service = ADService::new();
        
        let dn = service.create_organizational_unit("OU=Test,DC=sigmaos,DC=local".into(), "Test".into());
        
        assert!(service.get_object(&dn).is_some());
        assert_eq!(service.get_statistics().total_ous, 1);
    }
    
    #[test]
    fn test_create_user() {
        let mut service = ADService::new();
        
        let dn = service.create_user("testuser".into(), "Test User".into(), "test@sigmaos.local".into());
        
        assert!(service.get_user_by_dn(&dn).is_some());
        assert!(service.get_user_by_username("testuser").is_some());
        assert_eq!(service.get_statistics().total_users, 1);
    }
    
    #[test]
    fn test_create_group() {
        let mut service = ADService::new();
        
        let dn = service.create_group("Test Group".into(), GroupType::Security, GroupScope::Global);
        
        assert!(service.get_group_by_dn(&dn).is_some());
        assert!(service.get_group_by_name("Test Group").is_some());
        assert_eq!(service.get_statistics().total_groups, 1);
    }
    
    #[test]
    fn test_create_computer() {
        let mut service = ADService::new();
        
        let dn = service.create_computer("TEST-PC".into());
        
        assert!(service.get_object(&dn).is_some());
        assert_eq!(service.get_statistics().total_computers, 1);
    }
    
    #[test]
    fn test_add_user_to_group() {
        let mut service = ADService::new();
        service.initialize();
        
        let user_dn = service.create_user("testuser".into(), "Test User".into(), "test@sigmaos.local".into());
        let group_dn = service.create_group("Test Group".into(), GroupType::Security, GroupScope::Global);
        
        service.add_user_to_group(&user_dn, &group_dn);
        
        assert!(service.is_user_in_group(&user_dn, &group_dn));
        assert!(service.get_user_groups(&user_dn).contains(&group_dn));
        assert!(service.get_group_members(&group_dn).contains(&user_dn));
    }
    
    #[test]
    fn test_remove_user_from_group() {
        let mut service = ADService::new();
        service.initialize();
        
        let user_dn = service.create_user("testuser".into(), "Test User".into(), "test@sigmaos.local".into());
        let group_dn = service.create_group("Test Group".into(), GroupType::Security, GroupScope::Global);
        
        service.add_user_to_group(&user_dn, &group_dn);
        service.remove_user_from_group(&user_dn, &group_dn);
        
        assert!(!service.is_user_in_group(&user_dn, &group_dn));
    }
    
    #[test]
    fn test_search_users() {
        let mut service = ADService::new();
        service.create_user("testuser".into(), "Test User".into(), "test@sigmaos.local".into());
        service.create_user("admin".into(), "Administrator".into(), "admin@sigmaos.local".into());
        
        let results = service.search_users("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username, "testuser");
    }
    
    #[test]
    fn test_search_groups() {
        let mut service = ADService::new();
        service.create_group("Test Group".into(), GroupType::Security, GroupScope::Global);
        service.create_group("Admin Group".into(), GroupType::Security, GroupScope::Global);
        
        let results = service.search_groups("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].group_name, "Test Group");
    }
    
    #[test]
    fn test_get_objects_by_type() {
        let mut service = ADService::new();
        service.initialize();
        
        let users = service.get_objects_by_type(&ADObjectType::User);
        assert!(!users.is_empty());
        
        let groups = service.get_objects_by_type(&ADObjectType::Group);
        assert!(!groups.is_empty());
    }
    
    #[test]
    fn test_export_import() {
        let mut service = ADService::new();
        service.initialize();
        service.create_user("testuser".into(), "Test User".into(), "test@sigmaos.local".into());
        
        let export = service.export_data();
        
        let mut service2 = ADService::new();
        service2.import_data(export);
        
        assert_eq!(service2.get_statistics().total_users, service.get_statistics().total_users);
        assert!(service2.get_user_by_username("testuser").is_some());
    }
}
